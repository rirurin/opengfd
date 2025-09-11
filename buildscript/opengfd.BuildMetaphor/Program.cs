using opengfd.BuildCommon;

namespace opengfd.BuildMetaphor;
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
        };
    }
}

public class ProjectManager : ProjectManagerBase
{
    public override List<KeyValuePair<string, CodePackage>> GetProjects(ArgumentListBase arg, string RootPath)
    {
        return new List<KeyValuePair<string, CodePackage>>()
        {
            Register(new CSharpProject(arg, Path.Combine(RootPath, "metaphor.opengfd"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd-reloaded"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd-globals"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd-inspector"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd"))),
            Register(new RustCrate(arg, Path.Combine(RootPath, "opengfd-tests"))),
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

    public Executor(string[] args) : base(args) { }

    public override void Execute()
    {
        PrintInformation();
        // Copy Cri ADX links
        var opengfdBindings = Path.Combine(EnvManager["cri-adx-path"], "cri-adx-globals/middata/ext.rs");
        var libraryCrates = new List<string>() { "opengfd", "opengfd-inspector", "opengfd-reloaded", "opengfd-tests" };
        File.Copy(opengfdBindings, Path.Combine(ProjectManager["opengfd-reloaded"].RootPath, "src/adx.rs"), true);

        List<String> FeatureList = [ "v2-core", "reloaded", "image_loader", "serialize" ];
        foreach (var Feature in FeatureList) {
            ((RustCrate)ProjectManager["opengfd-globals"]).Features.Add(Feature);
        }
            
        // Build OpenGFD globals crate
        if (!ArgList["SkipGlobals"].Enabled)
        {
            string GetGlobalBindingPath(string Package)
            {
                var variant = Package.Equals("opengfd") switch { true => "self.rs", false => "ext.rs" };
                return Path.Combine(ProjectManager["opengfd-globals"].RootPath, "middata", variant);
            }

            ProjectManager["opengfd-globals"].Build();
            // Copy OpenGFD globals and functions to library crates + Reloaded crate
            foreach (string crate in libraryCrates)
                File.Copy(GetGlobalBindingPath(crate), Path.Combine(ProjectManager[crate].RootPath, "src/globals.rs"), true);
        }
        else Console.WriteLine($"Global crate compilation was skipped!");
        // Create riri_hook folder if it doesn't already exist
        Directory.CreateDirectory(Path.Combine(ProjectManager["opengfd-reloaded"].RootPath, "riri_hook"));
        // Build OpenGFD (Rust portion)
        ProjectManager["opengfd-reloaded"].Build();
        // Build OpenGFD (C# portion)
        ProjectManager["metaphor.opengfd"].Build();
        // Copy output files from target folder into Reloaded mod
        var reloadedDirectory = Path.Combine(Environment.GetEnvironmentVariable("RELOADEDIIMODS")!, "metaphor.opengfd");
        ((RustCrate)ProjectManager["opengfd-reloaded"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, RootPath, reloadedDirectory);
        ((RustCrate)ProjectManager["opengfd-globals"]).CopyOutputArtifacts(ArgList["Debug"].Enabled, RootPath, reloadedDirectory);
        // Copy mod files into Reloaded mod
        // Copied from Microsoft documentation :naosmiley:
        // https://learn.microsoft.com/en-us/dotnet/standard/io/how-to-copy-directories
        Utils.CopyDirectory(Path.Combine(ProjectManager["opengfd-reloaded"].RootPath, "data", "modfiles"), reloadedDirectory, true);
        PrintCompleted();
    }
}
public static class Program
{
    public static void Main(string[] args)
    {
        if (Environment.GetEnvironmentVariable("RELOADEDIIMODS") == null)
            throw new Exception("The environment variable RELOADEDIIMODS is not defined!");
        var exec = new Executor(args);
        exec.Execute();
    }
}