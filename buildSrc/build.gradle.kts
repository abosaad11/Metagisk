plugins {
    `kotlin-dsl`
}
repositories {
    google()
    mavenCentral()
}

gradlePlugin {
    plugins {
        register("MagiskPlugin") {
            id = "MagiskPlugin"
            implementationClass = "MagiskPlugin"
        }
    }
}

dependencies {
    implementation(kotlin("gradle-plugin", "1.8.10"))
    implementation("com.android.tools.build:gradle:8.0.0-beta03")
    implementation("com.android.tools:sdk-common:30.4.2")
    implementation("androidx.navigation:navigation-safe-args-gradle-plugin:2.5.3")
    implementation("org.lsposed.lsparanoid:gradle-plugin:0.5.1")
    implementation("org.eclipse.jgit:org.eclipse.jgit:6.4.0.202211300538-r")
}
