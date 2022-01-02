
const CRLIBM_INCLUDE_DIRECTORIES: &[&str] =
    &["crlibm", "crlibm/scs_lib"];

const CRLIBM_FILES: &[&str] =
    &["crlibm/crlibm_private.c",
      "crlibm/triple-double.c",
      "crlibm/exp-td.c",
      "crlibm/exp-td-standalone.c",
      "crlibm/expm1-standalone.c",
      "crlibm/expm1.c",
      "crlibm/log1p.c",
      "crlibm/rem_pio2_accurate.c",
      "crlibm/trigo_fast.c",
      "crlibm/trigo_accurate.c",
      "crlibm/trigpi.c",
      "crlibm/asincos.c",
      "crlibm/atan_fast.c",
      "crlibm/atan_accurate.c",
      "crlibm/csh_fast.c",
      "crlibm/scs_lib/scs_private.c",
      "crlibm/scs_lib/addition_scs.c",
      "crlibm/scs_lib/division_scs.c",
      // "crlibm/scs_lib/print_scs.c",
      "crlibm/scs_lib/double2scs.c",
      //"crlibm/scs_lib/zero_scs.c",
      "crlibm/scs_lib/multiplication_scs.c",
      "crlibm/scs_lib/scs2double.c" ];

fn in_appropriate_dir(file: &str) -> std::path::PathBuf {
    match std::env::var("OUT_DIR") {
        Err(_) => std::path::PathBuf::from(file),
        Ok(dir) => {
            use std::path::PathBuf;
            let mut path = PathBuf::new();
            path.push(dir);
            path.push(file);
            path
        }
    }
}

fn add_has_header_flag(build: &mut cc::Build, name: &str)
                       -> std::io::Result<()> {
    let tmp_file = in_appropriate_dir("build_has_header.c");
    use std::fs::File;
    use std::io::Write;
    { let mut file = File::create(&tmp_file)?;
      write!(file, "#include \"{}.h\"\nint main() {{ return 0; }}\n", name)?; }
    let mut local_build = cc::Build::new();
    let header_exists = local_build.warnings(false)
                                   .file(&tmp_file).try_expand().is_ok();
    if header_exists {
        let mut flag = String::from("HAVE_");
        flag.push_str(name.to_uppercase().as_str());
        flag.push_str("_H");
        build.define(flag.as_str(), None);
    }
    std::fs::remove_file(tmp_file).expect("Cannot remove file.");
    Ok(())
}

fn has_fpu_control() -> bool {
    let tmp_file = in_appropriate_dir("has_fpu_control.c");
    use std::fs::File;
    use std::io::Write;
    let file =
        File::create(&tmp_file)
        .and_then(|mut file| {
            write!(file, "#include \"fpu_control.h\"\nint main() {{\n\
                          unsigned long long cw;\n\
                          _FPU_SETCW(cw); return 0; }}\n")
                .and_then(|_| file.flush())
        });
    if file.is_err() { return false }
    let res = cc::Build::new().warnings(false)
        .file(&tmp_file).try_expand().is_ok();
    std::fs::remove_file(tmp_file).expect("Cannot remove file.");
    res
}

fn main() -> std::io::Result<()> {
    // Compile crlibm
    let mut build = cc::Build::new();
    // See "crlibm/configure.ac"
    let mut has_ia32_de = false;
    // "cygwin" || "mingw" || "mingw64"
    //   build.define("CRLIBM_TYPEOS_CYGWIN", None);
    if cfg!(target_os = "macos") || cfg!(target_os = "freebsd")
        || cfg!(target_os = "openbsd") || cfg!(target_os = "netbsd") {
            build.define("CRLIBM_TYPEOS_BSD", None);
        }
    if cfg!(target_arch = "powerpc") || cfg!(target_arch = "powerpc64") {
        build.define("CRLIBM_TYPECPU_POWERPC", None); }
    if cfg!(target_arch = "x86") {
        build.define("CRLIBM_TYPECPU_X86", None);
        has_ia32_de = true; }
    if cfg!(target_arch = "x86_64") {
        build.define("CRLIBM_TYPECPU_AMD64", None);
        has_ia32_de = true; }
    if cfg!(target_arch = "arm") || cfg!(target_arch = "aarch64") {
        panic!("CRlibm is not available on arm and aarch64.")
    }
    add_has_header_flag(&mut build, "fenv")?;
    add_has_header_flag(&mut build, "float")?;
    add_has_header_flag(&mut build, "inttypes")?;
    add_has_header_flag(&mut build, "stdlib")?;
    add_has_header_flag(&mut build, "strings")?;
    add_has_header_flag(&mut build, "unistd")?;
    let has_fpu_control = has_fpu_control();
    if has_fpu_control {
        build.define("CRLIBM_HAS_FPU_CONTROL", None);
    }
    build.define("SCS_NB_BITS", "30"); // default value
    build.define("SCS_NB_WORDS", "8"); // default value
    build.includes(CRLIBM_INCLUDE_DIRECTORIES);
    build.files(CRLIBM_FILES);
    // Select which files to compile for log.
    let use_hardware_de = has_ia32_de && has_fpu_control;
    if use_hardware_de {
        build.files(["crlibm/log-de.c", "crlibm/log2-td.c",
                     "crlibm/log10-td.c"]);
    } else {
        build.file("crlibm/log.c");
    }
    if cfg!(target_arch = "x86") && cfg!(target_os = "linux") /* elf */ {
        build.flag("-znotext");
    }

    // CRlibm produces lots of warnings.
    build.warnings(false);

    build.compile("crlibm");
    Ok(())
}
