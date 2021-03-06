pub use self::cargo_clean::{clean, CleanOptions};
pub use self::cargo_compile::{
    compile, compile_with_exec, compile_ws, resolve_all_features, CompileOptions,
};
pub use self::cargo_compile::{CompileFilter, FilterRule, LibRule, Packages};
pub use self::cargo_doc::{doc, DocOptions};
pub use self::cargo_fetch::{fetch, FetchOptions};
pub use self::cargo_generate_lockfile::generate_lockfile;
pub use self::cargo_generate_lockfile::update_lockfile;
pub use self::cargo_generate_lockfile::UpdateOptions;
pub use self::cargo_install::{install, install_list};
pub use self::cargo_new::{init, new, NewOptions, VersionControl};
pub use self::cargo_output_metadata::{output_metadata, ExportInfo, OutputMetadataOptions};
pub use self::cargo_package::{package, PackageOpts};
pub use self::cargo_pkgid::pkgid;
pub use self::cargo_read_manifest::{read_package, read_packages};
pub use self::cargo_run::run;
pub use self::cargo_test::{run_benches, run_tests, TestOptions};
pub use self::cargo_uninstall::uninstall;
pub use self::fix::{fix, fix_maybe_exec_rustc, FixOptions};
pub use self::lockfile::{load_pkg_lockfile, resolve_to_string, write_pkg_lockfile};
pub use self::registry::HttpTimeout;
pub use self::registry::{configure_http_handle, http_handle_and_timeout};
pub use self::registry::{http_handle, needs_custom_http_transport, registry_login, search};
pub use self::registry::{modify_owners, yank, OwnersOptions, PublishOpts};
pub use self::registry::{publish, registry_configuration, RegistryConfig};
pub use self::resolve::{
    add_overrides, get_resolved_packages, resolve_with_previous, resolve_ws, resolve_ws_with_opts,
};
pub use self::vendor::{vendor, VendorOptions};

mod cargo_clean;
mod cargo_compile;
mod cargo_doc;
mod cargo_fetch;
mod cargo_generate_lockfile;
mod cargo_install;
mod cargo_new;
mod cargo_output_metadata;
mod cargo_package;
mod cargo_pkgid;
mod cargo_read_manifest;
mod cargo_run;
mod cargo_test;
mod cargo_uninstall;
mod common_for_install_and_uninstall;
mod fix;
mod lockfile;
mod registry;
mod resolve;
pub mod tree;
mod vendor;
