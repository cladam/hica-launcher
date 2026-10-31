import "../src/catalogue"

fun catalogue_error(message: string) {
  eprintln(message)
  exit(1)
}

fun print_app(app: CatalogueApp) {
  println(app.name)
  println("  Repository: " + app.repo)
  println("  " + app.description)
}

fun run_catalogue(path: string) {
  match read_file(path) {
    Err(message) => catalogue_error("Cannot read catalogue: " + message),
    Ok(content) => {
      match catalogue_parse(content) {
        Err(message) => catalogue_error(message),
        Ok(apps) => {
          println("Applications: {length(apps)}")
          foreach(apps, print_app)
        }
      }
    }
  }
}

fun main() {
  match get_args() {
    [] => run_catalogue("catalogue.hml"),
    [path] => run_catalogue(path),
    _ => catalogue_error("Usage: catalogue_demo [catalogue.hml]")
  }
}
