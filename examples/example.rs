extern crate shell2batch;

fn main() {
    let script = shell2batch::convert(
        r#"
        export FILE1=file1
        export FILE2=file2

        #this is some test code
        cp ${FILE1} $FILE2

        #another
        mv file2 file3

        export MY_DIR=directory

        #flags are supported
        rm -Rf ${MY_DIR}

        unset MY_DIR
        "#,
    );

    assert_eq!(
        script,
        r#"
set FILE1=file1
set FILE2=file2

@REM this is some test code
xcopy %FILE1% %FILE2%

@REM another
move file2 file3

set MY_DIR=directory

@REM flags are supported
del /Q %MY_DIR%

set MY_DIR=
"#
    );

    println!("Script: {}", script);
}
