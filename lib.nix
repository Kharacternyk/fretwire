lib:
let
  inherit (lib)
    filter
    getAttrFromPath
    pipe
    readFile
    splitString
    ;
  read =
    file:
    pipe file [
      readFile
      (splitString "\n")
      (filter (line: line != ""))
    ];
in
{
  inherit read;
  import =
    set:
    let
      resolve = attribute: getAttrFromPath (splitString "." attribute) set;
    in
    file:
    pipe file [
      read
      (map resolve)
    ];
}
