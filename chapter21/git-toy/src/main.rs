mod raw;

use std::ffi::{CStr, CString};
use std::{mem, ptr};
use std::os::raw::{c_int, c_char};

fn check(activity: &'static str, status: c_int) -> c_int {
    if status.is_negative() {
        unsafe {
            let error = &*raw::giterr_last();
            println!(
                "error while {}: {} ({})",
                activity,
                CStr::from_ptr(error.message).to_string_lossy(),
                error.klass
            );
            std::process::exit(1);
        }
    }

    status
}

unsafe fn show_commit(commit: *const raw::git_commit) {
    let author = raw::git_commit_author(commit);

    let name = CStr::from_ptr((*author).name).to_string_lossy();
    let email = CStr::from_ptr((*author).email).to_string_lossy();
    println!("{} <{}>\n", name, email);

    let message = raw::git_commit_message(commit);
    println!("{}", CStr::from_ptr(message).to_string_lossy());
}

fn main() {
    let path = std::env::args().nth(1).expect("usage: git-toy PATH");
    let path = CString::new(path).expect("path contains null characters");

    unsafe {
        check("initializing libruary", raw::git_libgit2_init());

        let mut repo = ptr::null_mut();
        check(
            "opening repository",
            raw::git_repository_open(&mut repo, path.as_ptr()),
        );

        let c_name = b"HEAD\0".as_ptr() as *const c_char;
        let mut oid = mem::uninitialized();
        check(
            "looking up HEAD",
            raw::git_reference_name_to_id(&mut oid, repo, c_name),
        );

        let mut commit = ptr::null_mut();
        check(
            "looking up commit",
            raw::git_commit_lookup(&mut commit, repo, &oid),
        );

        show_commit(commit);

        raw::git_commit_free(commit);
        raw::git_repository_free(repo);

        check("shutting down libruary", raw::git_libgit2_shutdown());
    }
}