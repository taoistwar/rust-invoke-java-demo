package com.taoistwar.jni;

public class PrintLibraryPath {
  public String javaLibraryPath() {
    return System.getProperty("java.library.path");
  }
}
