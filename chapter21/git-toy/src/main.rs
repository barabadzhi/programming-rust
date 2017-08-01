extern crate libc;

mod git;

fn main() {
    let path = std::env::args_os().nth(1).expect("usage: git-toy PATH");

    let repo = git::Repository::open(&path).expect("opening repository");

    let commit_oid = repo.reference_name_to_id("HEAD").expect(
        "looking up 'HEAD' reference",
    );

    let commit = repo.find_commit(&commit_oid).expect("looking up commit");

    let autthor = commit.author();
    println!(
        "{} <{}>\n",
        autthor.name().unwrap_or("(none)"),
        autthor.email().unwrap_or("none")
    );

    println!("{}", commit.message().unwrap_or("(none)"));
}
