cmake_minimum_required(VERSION 3.14)

project(
        cameralot
        LANGUAGES CXX
)

# if top level project
if (CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    set(CMAKE_CXX_STANDARD 17)
    set(CMAKE_CXX_EXTENSIONS OFF)

    include(CTest)

    # FetchContent added in CMake 3.11, downloads during the configure step
    include(FetchContent)
    # FetchContent_MakeAvailable was not added until CMake 3.14; use our shim
    if(${CMAKE_VERSION} VERSION_LESS 3.14)
        include(cmake/add_FetchContent_MakeAvailable.cmake)
    endif()

    # support IDEs
    set_property(GLOBAL PROPERTY USE_FOLDERS ON)
endif ()

find_package(OpenCV REQUIRED COMPONENTS core videoio)
include_directories(${OpenCV_INCLUDE_DIRS})

file(
        GLOB SOURCES_LIST CONFIGURE_DEPENDS
        src/*.cpp
)

add_library(cameralot-capture ${SOURCES_LIST})
install(TARGETS cameralot-capture)

target_include_directories(
        cameralot-capture
        PUBLIC include
        PRIVATE opencv_core opencv_video opencv_imgproc
)

# OpenCV is required; link it here
target_link_libraries(
        cameralot-capture ${OpenCV_LIBS}
)

message(STATUS "Linked OpenCV")

if((CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME OR MODERN_CMAKE_BUILD_TESTING)
        AND BUILD_TESTING)
    add_subdirectory(cameralot-tests)
endif()