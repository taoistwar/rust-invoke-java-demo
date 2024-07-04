use jni::objects::{JObject, JString, JValue};
use jni::signature::{JavaType, Primitive};
use jni::sys::{jint, jobject};
use jni::{InitArgsBuilder, JNIEnv, JavaVM};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hello, world!");
    // JVM 初始化参数
    let jvm_args = InitArgsBuilder::new()
        .version(jni::JNIVersion::V8)
        .option("-Djava.class.path=D:\\workspace\\rust\\rust-alert\\java\\*")
        // .option("-Xcheck:jni")
        .option("-verbose:jni")
        .build()
        .unwrap();

    // 创建 JVM
    let jvm = JavaVM::new(jvm_args).unwrap();
    let mut env = jvm.attach_current_thread().unwrap();

    let file_class = env.find_class("java/io/File")?;
    // 获取静态字段
    let separator = env.get_static_field(file_class, "separator", "Ljava/lang/String;")?;
    let separator = env
        .get_string(&JString::from(separator.l()?))?
        .to_string_lossy()
        .to_string();
    println!("File.separator: {}", separator);
    assert_eq!(separator, format!("{}", std::path::MAIN_SEPARATOR));

    match (|| {
        let file_class = env.find_class("java/io/File")?;
        // 获取静态字段
        let separator = env.get_static_field(file_class, "separator", "Ljava/lang/String;")?;
        let separator = env
            .get_string(&JString::from(separator.l()?))?
            .to_string_lossy()
            .to_string();
        println!("File.separator: {}", separator);
        assert_eq!(separator, format!("{}", std::path::MAIN_SEPARATOR));
        // env.get_static_field_unchecked(class, field, ty)

        // 创建实例对象
        let file_name = JObject::from(env.new_string("./Cargo.toml")?);
        let file = env.new_object(
            "java/io/File",
            "(Ljava/lang/String;)V",
            &[JValue::from(&file_name)],
        )?;

        // 调用实例方法
        let abs = env.call_method(file, "getAbsolutePath", "()Ljava/lang/String;", &[])?;
        let abs_path = env
            .get_string(&JString::from(abs.l()?))?
            .to_string_lossy()
            .to_string();
        println!("abs_path: {}", abs_path);

        jni::errors::Result::Ok(())
    })() {
        Ok(_) => Ok(()),
        // 捕获异常
        Err(jni::errors::Error::JavaException) => {
            let except = env.exception_occurred().expect("exception_occurred");
            let err = env
                .call_method(except, "toString", "()Ljava/lang/String;", &[])
                .and_then(|e| {
                    Ok(env
                        .get_string(&JString::from(e.l()?))?
                        .to_string_lossy()
                        .to_string())
                })
                .unwrap_or_default();
            env.exception_clear().expect("clear exception");
            println!("call java exception occurred: {err}");
            Ok(())
        }
        Err(err) => {
            println!("call java error: {err:?}");
            Ok(())
        }
    }
}
