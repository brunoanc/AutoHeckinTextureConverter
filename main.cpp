#include <iostream>
#include <vector>
#include <cstdlib>
#include <system_error>
#include <filesystem>
#include "utils.hpp"

#ifdef _WIN32
#define SEPARATOR "\\"
const std::vector<std::string> tools{ "tools\\cuttlefish.exe", "tools\\cuttlefish.dll", "tools\\PVRTexLib.dll",
    "tools\\nvdecompress.exe", "tools\\nvtt.dll", "tools\\DivinityMachine.exe", "tools\\EternalTextureCompressor.exe" };
#else
#define SEPARATOR "/"
const std::vector<std::string> tools{ "tools/cuttlefish", "tools/libcuttlefish.so.2",
    "tools/libPVRTexLib.so", "tools/nvdecompress", "tools/DivinityMachine", "tools/EternalTextureCompressor" };
#endif

namespace fs = std::filesystem;

int main(int argc, char **argv)
{
    std::cout << "Auto Heckin' Texture Converter Rewrite by PowerBall253 :D" << std::endl;

    std::error_code ec;

    // Get executable's containing directory
    const std::string exeDir = fs::canonical(argv[0]).parent_path().string() + SEPARATOR;

    // Check if all required tools exist
    for (const auto& tool : tools) {
        if (!fs::is_regular_file(exeDir + tool, ec)) {
            std::cerr << "\n'" << tool.substr(tool.rfind(SEPARATOR) + 1) << "' not found! Did you extract everything in the tools folder?" << std::endl;
            return 1;
        }
    }

    // Display help if no arguments are provided
    if (argc == 1) {
        std::cout << "\nUsage:\n";
        std::cout << argv[0] << " [texture1] [texture2] [...]\n\n";
#ifdef _WIN32
        std::cout << "Alternatively, drag files onto this executable.\n" << std::endl;
#else
        std::cout << std::flush;
#endif
        return 1;
    }

    // Convert textures
    int failures = 0;

    for (int i = 1; i < argc; i++) {
        std::cout << std::endl;

        // Get texture's filename and stripped filename
        const std::string filePath(argv[i]);
        const std::string fileName = fs::path(argv[i]).filename().string();
        const std::string strippedFileName = fileName.substr(0, fileName.find_first_of(".$"));

        std::cout << "Converting '" << fs::path(argv[i]).string() << "'..." << std::endl;

        // Check if given path exists and is a file
        if (!fs::is_regular_file(filePath, ec)) {
            std::cerr << "ERROR: " << argv[i] << " was not found." << std::endl;
            failures += 1;
            continue;
        }

        // Get target format
        std::string format = "BC1_RGBA";

        if (fileName.find("$bc7") != std::string::npos)
            format = "BC7";
        else if (endsWith(strippedFileName, "_n") || endsWith(strippedFileName, "_Normal"))
            format = "BC5";

        // If texture is DDS, decompress it first
        if (fs::path(fileName).extension() == ".dds") {
            // Build nvdecompress's command
            // nvdecompress [input texture] [output texture]
#ifdef _WIN32
            const std::string nvdecompressPath = "\"" + exeDir + "tools\\nvdecompress.exe\"";
            const std::string nvdecompressCommand = "\"" + nvdecompressPath + " \"" + filePath + "\" \"" + filePath + ".tga\" >nul\"";
#else
            const std::string nvdecompressPath = "\"" + exeDir + "tools/nvdecompress\"";
            const std::string nvdecompressCommand = nvdecompressPath + " \"" + filePath + "\" \"" + filePath + ".tga\" > /dev/null";
#endif

            // Run nvdecompress
            if (std::system(nvdecompressCommand.c_str()) != 0) {
                std::cerr << "ERROR: Failed to decompress texture using nvdecompress." << std::endl;
                failures += 1;
                continue;
            }

            // Recompress texture
            // Build cuttlefish's command
            // cuttlefish --input [input texture] --mipmap --quality lowest --format [format] --output [output texture]
#ifdef _WIN32
            const std::string cuttlefishPath = "\"" + exeDir + "tools\\cuttlefish.exe\"";
            const std::string cuttlefishCommand = "\"" + cuttlefishPath + " --input \"" + filePath + ".tga\" --mipmap --quality lowest --format " + format + " --output \"" + filePath + ".dds\" >nul\"";
#else
            const std::string cuttlefishPath = "\"" + exeDir + "tools/cuttlefish\"";
            const std::string cuttlefishCommand = cuttlefishPath + " --input \"" + filePath + ".tga\" --mipmap --quality lowest --format " + format + " --output \"" + filePath + ".dds\" > /dev/null";
#endif

            // Run cuttlefish
            if (std::system(cuttlefishCommand.c_str()) != 0) {
                std::cerr << "ERROR: Failed to convert texture to " << format << " using cuttlefish." << std::endl;
                failures += 1;
                continue;
            }

            // Remove decompressed TARGA file
            fs::remove(filePath + ".tga", ec);
        }
        else {
            // Compress texture
            // Build cuttlefish's command
            // cuttlefish --input [input texture] --mipmap --quality lowest --format [format] --output [output texture]
#ifdef _WIN32
            const std::string cuttlefishPath = "\"" + exeDir + "tools\\cuttlefish.exe\"";
            const std::string cuttlefishCommand = "\"" + cuttlefishPath + " --input \"" + filePath + "\" --mipmap --quality lowest --format " + format + " --output \"" + filePath + ".dds\" >nul\"";
#else
            const std::string cuttlefishPath = "\"" + exeDir + "tools/cuttlefish\"";
            const std::string cuttlefishCommand = cuttlefishPath + " --input \"" + filePath + "\" --mipmap --quality lowest --format " + format + " --output \"" + filePath + ".dds\" > /dev/null";
#endif

            // Run cuttlefish
            if (std::system(cuttlefishCommand.c_str()) != 0) {
                std::cerr << "ERROR: Failed to convert texture to " << format << " using cuttlefish." << std::endl;
                failures += 1;
                continue;
            }
        }

        // Build DivinityMachine's command
        // DivinityMachine [input texture]
#ifdef _WIN32
        std::string divinityPath = "\"" + exeDir + "tools\\DivinityMachine.exe\"";
        const std::string divinityCommand = "\"" + divinityPath + " \"" + filePath + ".dds\" >nul\"";
#else
        std::string divinityPath = "\"" + exeDir + "tools/DivinityMachine\"";
        const std::string divinityCommand = divinityPath + " \"" + filePath + ".dds\" > /dev/null";
#endif

        // Run DivinityMachine
        if (std::system(divinityCommand.c_str()) != 0) {
            std::cerr << "ERROR: Failed to convert texture to TGA using DivinityMachine." << std::endl;
            fs::remove(filePath + ".dds", ec);
            failures += 1;
            continue;
        }

        // Remove DivinityMachine's input file
        fs::remove(filePath + ".dds", ec);

        // Get DivinityMachine's output filename
        std::string divinityExtension = ".tga";
        const std::string strippedPropertiesFileName = fs::path(fileName).stem().string().substr(0, fileName.find_first_of('$'));

        if (endsWith(strippedPropertiesFileName, ".png"))
            divinityExtension = ".png";

        // Build EternalTextureCompressor's command
        // EternalTextureCompressor [input BIMAGE]
#ifdef _WIN32
        std::string compressorPath = "\"" + exeDir + "tools\\EternalTextureCompressor.exe\"";
        const std::string compressorCommand = "\"" + compressorPath + " \"" + filePath + divinityExtension + "\" >nul\"";
#else
        std::string compressorPath = "\"" + exeDir + "tools/EternalTextureCompressor\"";
        const std::string compressorCommand = compressorPath + " \"" + filePath + divinityExtension + "\" > /dev/null";
#endif

        // Run EternalTextureCompressor
        if (std::system(compressorCommand.c_str()) != 0) {
            std::cerr << "ERROR: Failed to compress texture using EternalTextureCompressor." << std::endl;
            fs::remove(filePath + divinityExtension, ec);
            failures += 1;
            continue;
        }

        // Rename the output file
        std::string newFilePath;

        if (fileName.find('$') != std::string::npos)
            newFilePath = fs::path(filePath).replace_extension("").string();
        else
            newFilePath = fs::path(filePath).replace_extension(divinityExtension).string();

        fs::rename(filePath + divinityExtension, newFilePath, ec);

        // Check for errors in file renaming
        if (ec.value() != 0) {
            std::cerr << "ERROR: Failed to rename " << filePath + divinityExtension << "." << std::endl;
            fs::remove(filePath + divinityExtension, ec);
            failures += 1;
            continue;
        }

        std::cout << "Successfully converted " << fileName << " into " << fs::path(newFilePath).filename().string() << "." << std::endl;
    }

    // Exit
    std::cout << "\nDone." << std::endl;
    pressAnyKey();
    return failures;
}
