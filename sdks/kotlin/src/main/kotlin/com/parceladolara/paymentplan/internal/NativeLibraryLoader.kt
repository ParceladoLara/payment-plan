package com.parceladolara.paymentplan.internal

import java.io.File
import java.io.FileOutputStream
import java.io.InputStream

/**
 * Utility class for loading native libraries from JAR resources.
 *
 * This class handles the extraction and loading of platform-specific native libraries that are
 * embedded as resources within the JAR file.
 *
 * Supported platforms: Windows and Linux
 *
 * This loader integrates with UniFFI by setting the appropriate system property that UniFFI uses to
 * locate the native library.
 */
internal object NativeLibraryLoader {

    private var isLoaded = false
    private var libraryPath: String? = null

    /**
     * Loads the payment plan native library from JAR resources.
     *
     * The library will be extracted to a temporary directory and then loaded. This method ensures
     * the library is only loaded once and sets the system property that UniFFI uses to locate the
     * library.
     */
    @Synchronized
    fun loadLibrary() {
        if (isLoaded) {
            return
        }

        val libraryName = getLibraryName()
        val resourcePath = getResourcePath(libraryName)

        try {
            // Try to load from JAR resources first
            val resourcePath = getResourcePath(libraryName)
            val inputStream = javaClass.getResourceAsStream(resourcePath)

            if (inputStream != null) {
                val libraryPath = loadFromResource(inputStream)
                // Set system property to help UniFFI find the library
                System.setProperty(
                        "uniffi.component.payment_plan_uniffi.libraryOverride",
                        libraryPath
                )
            } else {
                // Fallback to file system loading (development mode)
                loadFromFileSystem(libraryName)
            }
        } catch (e: Exception) {
            throw RuntimeException("Failed to load native library: $libraryName", e)
        }
    }

    /** Determines the platform-specific library name. */
    private fun getLibraryName(): String {
        val osName = System.getProperty("os.name").lowercase()
        return when {
            osName.contains("windows") -> "payment_plan_uniffi.dll"
            osName.contains("linux") -> "libpayment_plan_uniffi.so"
            else ->
                    throw UnsupportedOperationException(
                            "Unsupported operating system: $osName. Only Windows and Linux are supported."
                    )
        }
    }

    /** Determines the resource path based on the platform. */
    private fun getResourcePath(libraryName: String): String {
        val osName = System.getProperty("os.name").lowercase()
        return when {
            osName.contains("windows") -> "/native/windows/$libraryName"
            osName.contains("linux") -> "/native/linux/$libraryName"
            else ->
                    throw UnsupportedOperationException(
                            "Unsupported operating system: $osName. Only Windows and Linux are supported."
                    )
        }
    }

    /** Loads the library from JAR resources by extracting it to a temporary file. */
    private fun loadFromResource(inputStream: InputStream): String {
        // Create a temporary file
        val tempFile = File.createTempFile("payment_plan_uniffi", getLibraryExtension())
        tempFile.deleteOnExit()

        // Copy the library from resources to the temporary file
        inputStream.use { input ->
            FileOutputStream(tempFile).use { output -> input.copyTo(output) }
        }

        // Set executable permissions (important for Unix-like systems)
        tempFile.setExecutable(true)
        tempFile.setReadable(true)

        // Load the library
        System.load(tempFile.absolutePath)

        println("Loaded native library from JAR resources: ${tempFile.absolutePath}")

        // Return the path for UniFFI integration
        return tempFile.absolutePath
    }

    /** Fallback method to load library from file system (development mode). */
    private fun loadFromFileSystem(libraryName: String) {
        // Try to load using JNA's standard mechanism
        System.setProperty(
                "jna.library.path",
                System.getProperty("jna.library.path", "") +
                        File.pathSeparator +
                        System.getProperty("user.dir") +
                        "/_internal"
        )

        // For development, try loading from _internal directory
        val devPath = File("_internal/$libraryName")
        if (devPath.exists()) {
            System.load(devPath.absolutePath)
            println("Loaded native library from development path: ${devPath.absolutePath}")
        } else {
            // Last resort: try system library loading
            val baseName = libraryName.replace("lib", "").replace(".so", "").replace(".dll", "")
            System.loadLibrary(baseName)
            println("Loaded native library using system loader: $baseName")
        }
    }

    /** Gets the appropriate file extension for the current platform. */
    private fun getLibraryExtension(): String {
        val osName = System.getProperty("os.name").lowercase()
        return when {
            osName.contains("windows") -> ".dll"
            osName.contains("linux") -> ".so"
            else ->
                    throw UnsupportedOperationException(
                            "Unsupported operating system: $osName. Only Windows and Linux are supported."
                    )
        }
    }
}
