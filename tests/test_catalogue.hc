import "../src/catalogue"

fun fixture_app(repo: string) {
  "@app \{\nname: \"Fixture\"\nrepo: \"" + repo + "\"\ndescription: \"Test fixture only\"\n\}"
}

fun catalogue_rejected(content: string, expected: string) {
  match catalogue_parse(content) {
    Err(message) => contains(message, expected),
    Ok(_) => false
  }
}

test "empty catalogue is valid" {
  let parsed = catalogue_parse("// No confirmed apps yet\n")
  let empty = match parsed {
    Ok(apps) => length(apps) == 0,
    Err(_) => false
  }
  assert(empty)
}

test "valid app fields are decoded" {
  let parsed = catalogue_parse(fixture_app("fixture-owner/sample_app"))
  let valid = match parsed {
    Ok([app]) => app.name == "Fixture" && app.repo == "fixture-owner/sample_app" && app.description == "Test fixture only",
    _ => false
  }
  assert(valid)
}

test "catalogue preserves entry order" {
  let parsed = catalogue_parse(fixture_app("fixture/first") + "\n" + fixture_app("fixture/second"))
  let ordered = match parsed {
    Ok([first, second]) => first.repo == "fixture/first" && second.repo == "fixture/second",
    _ => false
  }
  assert(ordered)
}

test "duplicate repositories are rejected ignoring case" {
  assert(catalogue_rejected(fixture_app("Fixture/App") + "\n" + fixture_app("fixture/app"), "Duplicate GitHub repository"))
}

test "invalid later entry rejects the entire catalogue" {
  assert(catalogue_rejected(fixture_app("fixture/app") + "\n" + fixture_app("not-a-repo"), "Invalid GitHub owner/repo"))
}

test "malformed repository identifiers are rejected" {
  let invalid = ["", "owner", "owner/", "/repo", "owner/repo/extra", "https://github.com/owner/repo", "owner name/repo", "-owner/repo", "owner-/repo", "ow--ner/repo", "owner/.", "owner/..", "owner/repo?x=1"]
  assert(all(invalid, (repo) => !catalogue_repo_valid(repo)))
}

test "missing required fields are rejected" {
  assert(catalogue_rejected("@app \{\nrepo: \"fixture/app\"\ndescription: \"Test\"\n\}", "missing name"))
  assert(catalogue_rejected("@app \{\nname: \"Fixture\"\ndescription: \"Test\"\n\}", "missing repo"))
  assert(catalogue_rejected("@app \{\nname: \"Fixture\"\nrepo: \"fixture/app\"\n\}", "missing description"))
}

test "blank and non-string fields are rejected" {
  assert(catalogue_rejected("@app \{\nname: \"   \"\nrepo: \"fixture/app\"\ndescription: \"Test\"\n\}", "must not be blank: name"))
  assert(catalogue_rejected("@app \{\nname: 42\nrepo: \"fixture/app\"\ndescription: \"Test\"\n\}", "must be a string: name"))
  assert(catalogue_rejected("@app \{\nname: \"Fixture\"\nrepo: \"fixture/app\"\ndescription: \"\"\n\}", "must not be blank: description"))
}

test "surrounding whitespace is rejected" {
  assert(catalogue_rejected(fixture_app(" fixture/app"), "surrounding whitespace: repo"))
}

test "unexpected catalogue structure is rejected" {
  assert(catalogue_rejected("@other \{\}", "only @app elements"))
  assert(catalogue_rejected("apps: []", "only @app elements"))
  assert(catalogue_rejected("@app(enabled: true) \{\}", "only @app elements"))
  assert(catalogue_rejected("@app \{\nname: \"Fixture\"\nrepo: \"fixture/app\"\ndescription: \"Test\"\nextra: \"no\"\n\}", "Unknown app field: extra"))
}

test "duplicate app fields are rejected" {
  assert(catalogue_rejected("@app \{\nname: \"First\"\nname: \"Second\"\nrepo: \"fixture/app\"\ndescription: \"Test\"\n\}", "Duplicate"))
}

test "invalid HML reports an error" {
  assert(catalogue_rejected("@app \{", "Invalid catalogue HML"))
}