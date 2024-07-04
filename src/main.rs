use jni::objects::{JObject, JString, JValue};
use jni::signature::{JavaType, Primitive};
use jni::sys::{jint, jobject};
use jni::{InitArgsBuilder, JNIEnv, JavaVM};

mod invoke_jdk;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // JVM 初始化参数
    let jvm_args = InitArgsBuilder::new()
        .version(jni::JNIVersion::V8)
        .option("-Djava.class.path=./java")
        // .option("-Xcheck:jni")
        .option("-verbose:jni")
        .build()
        .unwrap();

    // 创建 JVM
    let jvm = JavaVM::new(jvm_args).unwrap();
    let mut env = jvm.attach_current_thread().unwrap();

    match (|| {
        let file_class = env.find_class("com/taoistwar/jni/PrintLibraryPath")?;
        println!("{:?}", file_class);
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
