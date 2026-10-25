lib:
let inherit (lib) pipe filter readFile splitString getAttrFromPath;
  read = file: pipe file [
    readFile
    (splitString "\n")
    (filter (line: line != ""))
  ];
in
{
  inherit read;
  import = set: file:
    map (attribute: getAttrFromPath (splitString "." attribute) set) (read file);
}
