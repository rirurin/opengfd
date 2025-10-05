using opengfd.BuildCommon;

namespace opengfd.BuildP5R;

public class SkipGlobals : Argument
{
    public override void HandleParams(string[] args)
    {
        Enabled = args[0].ToLower() switch
        {
            "true" => true,
            "false" => false,
            _ => throw new Exception($"Expected a boolean value, got {args[0]} instead")
        };
    }

    public override int GetParamCount() => 1;
}

public class Timings : Argument
{
    public override void HandleParams(string[] args)
    {
        Enabled = args[0].ToLower() switch
        {
            "true" => true,
            "false" => false,
            _ => throw new Exception($"Expected a boolean value, got {args[0]} instead")
        };
    }

    public override int GetParamCount() => 1;
}

public class Publish : Argument
{
    public override void HandleParams(string[] args)
    {
        Enabled = args[0].ToLower() switch
        {
            "true" => true,
            "false" => false,
            _ => throw new Exception($"Expected a boolean value, got {args[0]} instead")
        };
    }

    public override int GetParamCount() => 1;
}

public class ArgumentList : ArgumentListBase
{
    public ArgumentList(string[] args) : base(args) { }

    protected override Dictionary<string, Argument> SetArguments()
    {
        return new()
        {
            { "Debug", new Debug() },
            { "SkipGlobals", new SkipGlobals() },
            { "Timings", new Timings() },
            { "Publish", new Publish() }
        };
    }
}

public class ProjectManager : ProjectManagerBase
{
    public override List<KeyValuePair<string, CodePackage>> GetProjects(ArgumentListBase arg, string RootPath)
    {
        return new List<KeyValuePair<string, CodePackage>>()
        {
            Register(new CSharpProject(arg, Path.Combine(RootPath, "p5rpc.opengfd"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd-reloaded-p5r"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd-globals"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd"))),
        };
    }
    public ProjectManager(ArgumentList arg, string RootPath) : base(arg, RootPath) { }
}

public class Executor : ExecutorBase<ArgumentList, ProjectManager>
{
    public override string BuildType
    {
        get => "CLIENT";
    }

    public Executor(string[] args, string PackageFolder) : base(args, PackageFolder) { }

    public override void Execute()
    {
        if (ArgList["Publish"].Enabled)
        {
            PublishState.Cleanup();
            PublishState.CheckTools();
        }
        PrintInformation();
        // Create riri_hook folder if it doesn't already exist
        Directory.CreateDirectory(Path.Combine(ProjectManager["opengfd-reloaded-p5r"].RootPath, "riri_hook"));
        // Copy Cri ADX links
        var libraryCrates = new List<string>() { "opengfd", "opengfd-reloaded-p5r" };
        if (ArgList["Publish"].Enabled)
        {
            // Copy from remote repository - make sure that dependencies have committed up-to-date bindings first!
            var linkFile = Utils.DownloadFile(
                "https://raw.githubusercontent.com/rirurin/cri-adx-rs/refs/heads/main/cri-adx-globals/middata/ext.rs");
            File.WriteAllBytes(Path.Combine(ProjectManager["opengfd-reloaded-p5r"].RootPath, "src/adx.rs"), linkFile);
        }
        else
        {
            // Copy from local environment
            var opengfdBindings = Path.Combine(EnvManager["cri-adx-path"], "cri-adx-globals/middata/ext.rs");
            File.Copy(opengfdBindings, Path.Combine(ProjectManager["opengfd-reloaded-p5r"].RootPath, "src/adx.rs"), true);   
        }
        List<string> FeatureList = [ "v1-core", "reloaded", "image_loader", "serialize" ];
        foreach (var Feature in FeatureList) {
            ((RustCrate)ProjectManager["opengfd-globals"]).Features.Add(Feature);
        }
        ((RustCrate)ProjectManager["opengfd-globals"]).UseDefaultFeatures = false;

        // Build OpenGFD globals crate
        if (!ArgList["SkipGlobals"].Enabled)
        {
            string GetGlobalBindingPath(string Package)
            {
                var variant = Package.Equals("opengfd") switch { true => "self_xrd744.rs", false => "ext_xrd744.rs" };
                return Path.Combine(ProjectManager["opengfd-globals"].RootPath, "middata", variant);
            }

            ProjectManager["opengfd-globals"].Build();
            // Copy OpenGFD globals and functions to library crates + Reloaded crate
            foreach (string crate in libraryCrates)
                File.Copy(GetGlobalBindingPath(crate), Path.Combine(ProjectManager[crate].RootPath, "src/globals_xrd744.rs"), true);
        }
        else Console.WriteLine($"Global crate compilation was skipped!");
        // Create riri_hook folder if it doesn't already exist
        Directory.CreateDirectory(Path.Combine(ProjectManager["opengfd-reloaded-p5r"].RootPath, "riri_hook"));
        // Build OpenGFD (Rust portion)
        ProjectManager["opengfd-reloaded-p5r"].Build();
        // Build OpenGFD (C# portion)
        if (ArgList["Publish"].Enabled)
        {
            ((CSharpProject)ProjectManager["p5rpc.opengfd"]).PublishBuildDirectory = PublishState.PublishBuildDirectory;
            ((CSharpProject)ProjectManager["p5rpc.opengfd"]).TempDirectory = PublishState.TempDirectoryBuild;
            Directory.CreateDirectory(PublishState.PublishBuildDirectory);
            ((RustCrate)ProjectManager["opengfd-reloaded-p5r"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, 
                RootPath, PublishState.PublishBuildDirectory);
            ((RustCrate)ProjectManager["opengfd-globals"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, 
                RootPath, PublishState.PublishBuildDirectory);
            var modFiles = Path.Combine(ProjectManager["opengfd-reloaded-p5r"].RootPath, "data", "modfiles");
            if (Directory.Exists(modFiles))
            {
                Utils.CopyDirectory(modFiles, PublishState.PublishBuildDirectory, true);
            }
        }
        ProjectManager["p5rpc.opengfd"].Build();
        if (ArgList["Publish"].Enabled)
        {
            PublishState.CreateArtifacts("p5rpc.opengfd");
        }
        else
        {
            // Copy output files from target folder into Reloaded mod
            var reloadedDirectory = Path.Combine(Environment.GetEnvironmentVariable("RELOADEDIIMODS")!, "p5rpc.opengfd");
            ((RustCrate)ProjectManager["opengfd-reloaded-p5r"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, RootPath, reloadedDirectory);
            ((RustCrate)ProjectManager["opengfd-globals"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, RootPath, reloadedDirectory);
            var modFiles = Path.Combine(ProjectManager["opengfd-reloaded-p5r"].RootPath, "data", "modfiles");
            if (Directory.Exists(modFiles))
            {
                Utils.CopyDirectory(modFiles, reloadedDirectory, true);
            }   
        }
        PrintCompleted();
    }
}

public static class Program
{
    public static void Main(string[] args)
    {
        if (Environment.GetEnvironmentVariable("RELOADEDIIMODS") == null)
            throw new Exception("The environment variable RELOADEDIIMODS is not defined!");
        var exec = new Executor(args, "P5R");
        exec.Execute();
    }
}