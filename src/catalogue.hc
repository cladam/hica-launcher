import "hml"

pub struct CatalogueApp {
  name: string,
  repo: string,
  description: string
}

pub fun catalogue_identifier(text: string, alphabet: string) {
  length(text) > 0 && all(chars(text), (character) => contains(alphabet, char_to_string(character)))
}

pub fun catalogue_repo_valid(repo: string) {
  let letters = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"
  match split(repo, "/") {
    [owner, project] => {
      catalogue_identifier(owner, letters + "-") && length(owner) <= 39 &&
      !starts_with(owner, "-") && !ends_with(owner, "-") && !contains(owner, "--") &&
      catalogue_identifier(project, letters + "-._") && length(project) <= 100 &&
      project != "." && project != ".."
    },
    _ => false
  }
}

pub fun catalogue_fields_valid(nodes: list<HmlNode>, seen: list<string>) {
  match nodes {
    [] => Ok(true),
    [NComment(_), ..rest] => catalogue_fields_valid(rest, seen),
    [NProp(key, HStr(text)), ..rest] => {
      if key != "name" && key != "repo" && key != "description" {
        Err("Unknown app field: " + key)
      } else if any(seen, (existing) => existing == key) {
        Err("Duplicate app field: " + key)
      } else if trim(text) == "" {
        Err("App field must not be blank: " + key)
      } else if text != trim(text) {
        Err("App field has surrounding whitespace: " + key)
      } else {
        catalogue_fields_valid(rest, [key] + seen)
      }
    },
    [NProp(key, _), .._] => Err("App field must be a string: " + key),
    _ => Err("App bodies may contain only name, repo, and description properties")
  }
}

pub fun catalogue_required_string(body: list<HmlNode>, key: string, context: string) {
  match as_str(hml_get(body, key)) {
    None => Err("App is missing " + key + context),
    Some(text) => Ok(text)
  }
}

pub fun catalogue_decode_app(body: list<HmlNode>) : result<CatalogueApp, string> {
  let validated = catalogue_fields_valid(body, [])?
  let name = catalogue_required_string(body, "name", "")?
  let repo = catalogue_required_string(body, "repo", ": " + name)?
  let description = catalogue_required_string(body, "description", ": " + name)?
  if catalogue_repo_valid(repo) {
    Ok(CatalogueApp { name: name, repo: repo, description: description })
  } else {
    Err("Invalid GitHub owner/repo: " + repo)
  }
}

pub fun catalogue_decode_nodes(nodes: list<HmlNode>, seen: list<string>) {
  match nodes {
    [] => Ok([]),
    [NComment(_), ..rest] => catalogue_decode_nodes(rest, seen),
    [NElem(HElement("app", [], body)), ..rest] => {
      match catalogue_decode_app(body) {
        Err(message) => Err(message),
        Ok(app) => {
          let repo_key = to_lower(app.repo)
          if any(seen, (existing) => existing == repo_key) {
            Err("Duplicate GitHub repository: " + app.repo)
          } else {
            match catalogue_decode_nodes(rest, [repo_key] + seen) {
              Err(message) => Err(message),
              Ok(apps) => Ok([app] + apps)
            }
          }
        }
      }
    },
    _ => Err("Catalogue may contain only @app elements without attributes")
  }
}

pub fun catalogue_parse(content: string) {
  match hml_parse(content) {
    Err(message) => Err("Invalid catalogue HML: " + message),
    Ok(nodes) => catalogue_decode_nodes(nodes, [])
  }
}
