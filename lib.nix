lib:
let inherit (lib) filter readFile splitString getAttrFromPath;
in
{
  import = set: file:
    map (attribute: getAttrFromPath (splitString "." attribute) set) (
      filter (attribute: attribute != "") (
        splitString "\n" (
          readFile file
        )
      )
    );
}
