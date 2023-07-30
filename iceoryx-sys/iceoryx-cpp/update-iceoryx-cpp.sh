#! /bin/bash

shopt -s expand_aliases

# colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
CYAN='\033[0;36m'
COLOR_OFF='\033[0m'

GIT_TOPLEVEL=$(git rev-parse --show-toplevel)
ICEORYX_CPP_BASE_DIR=$GIT_TOPLEVEL/iceoryx-sys/iceoryx-cpp
ICEORYX_TAG="v0.0.0"

DO_BUILD_ONLY=false
DO_PREPARATORY_WORK=false
DO_RELEASE=false

OPTION_COUNTER=0
while (( "$#" )); do
    ((OPTION_COUNTER+=1))
    if [[ $OPTION_COUNTER -gt 1 ]]; then
        echo -e "${RED}Error:${COLOR_OFF} Too many arguments specified! Try '--help' for more information."
        exit -1
    fi

    case $1 in
        -h|--help)
            echo "Script to update bundled iceoryx C++ source archive"
            echo ""
            echo "Usage: update-iceoryx-cpp.sh [option]"
            echo "Only one option at a time is allowed!"
            echo "Options:"
            echo "    -t, --tag <TAG>                   Updates the bundled source archive with"
            echo "                                      the given <TAG>"
            echo "    -h, --help                        Prints this help"
            echo ""
            echo "Example:"
            echo "    update-iceoryx-cpp.sh --tag v2.0.0"
            exit 0
            ;;
        -t|--tag)
            if [[ $# -ne 2 ]]; then
                echo -e "${RED}Error:${COLOR_OFF} No parameter specified! Try '--help' for more information."
            fi
            ICEORYX_TAG="$2"

            shift 2
            ;;
        *)
            echo "Invalid argument '$1'. Try '--help' for options."
            exit -1
            ;;
    esac
done

if [[ $OPTION_COUNTER -eq 0 ]]; then
    echo -e "${RED}Error:${COLOR_OFF} No arguments specified! Try '--help' for more information."
    exit -1
fi

####################
# cd into base dir #
####################

cd $ICEORYX_CPP_BASE_DIR

echo -e "${CYAN}Info:${COLOR_OFF} Entering '$(pwd)'"

#####################
# Creating temp dir #
#####################

ICEORYX_TEMP_DIR=iox-temp

rm -rf ${ICEORYX_TEMP_DIR}
mkdir -p ${ICEORYX_TEMP_DIR}
cd ${ICEORYX_TEMP_DIR}

#######################
# Downloading archive #
#######################

ICEORYX_ARCHIVE="${ICEORYX_TAG}.tar.gz"
ICEORYX_ARCHIVE_LINK="https://github.com/eclipse-iceoryx/iceoryx/archive/refs/tags/${ICEORYX_ARCHIVE}"

echo -e "${CYAN}Info:${COLOR_OFF} Fetching '${ICEORYX_ARCHIVE_LINK}'"

wget ${ICEORYX_ARCHIVE_LINK} --quiet --show-progress

if [ $? -ne 0 ]; then
    echo -e "${RED}Error:${COLOR_OFF} Could not fetch '${ICEORYX_ARCHIVE_LINK}'"
    exit -1
fi

##########################
# Extracting new archive #
##########################

echo -e "${CYAN}Info:${COLOR_OFF} Extracting new source archive to '${ICEORYX_TAG}'"

mkdir -p ${ICEORYX_TAG}
tar -xf ${ICEORYX_ARCHIVE} -C ${ICEORYX_TAG} --strip-components=1

#################
# Strip archive #
#################

echo -e "${CYAN}Info:${COLOR_OFF} Stripping source archive from unnecessary files"

rm -rf ${ICEORYX_TAG}/.clang-format
rm -rf ${ICEORYX_TAG}/.clang-tidy
rm -rf ${ICEORYX_TAG}/.codecov.yml
rm -rf ${ICEORYX_TAG}/.gitattributes
rm -rf ${ICEORYX_TAG}/.github
rm -rf ${ICEORYX_TAG}/.gitignore
rm -rf ${ICEORYX_TAG}/cmake
rm -rf ${ICEORYX_TAG}/doc
rm -rf ${ICEORYX_TAG}/iceoryx_binding_c
rm -rf ${ICEORYX_TAG}/iceoryx_dds
rm -rf ${ICEORYX_TAG}/iceoryx_examples
rm -rf ${ICEORYX_TAG}/iceoryx_hoofs/test
rm -rf ${ICEORYX_TAG}/iceoryx_integrationtest
rm -rf ${ICEORYX_TAG}/iceoryx_meta
rm -rf ${ICEORYX_TAG}/iceoryx_posh/test
rm -rf ${ICEORYX_TAG}/mkdocs.yml
rm -rf ${ICEORYX_TAG}/tools


#######################
# Re-compress archive #
#######################

echo -e "${CYAN}Info:${COLOR_OFF} Re-compressing '${ICEORYX_TAG}' into '${ICEORYX_ARCHIVE}'"

# restore modified timestamp to prevent changing the archive for repetitive updates to the same tag
tar -czf ${ICEORYX_ARCHIVE} ${ICEORYX_TAG} --mtime ./${ICEORYX_TAG}/VERSION

####################
# Back to base dir #
####################

cd $ICEORYX_CPP_BASE_DIR

###################
# Replace archive #
###################

echo -e "${CYAN}Info:${COLOR_OFF} Replace archive"

rm -rf v*\.tar\.gz
mv ${ICEORYX_TEMP_DIR}/${ICEORYX_ARCHIVE} .

##########################
# Remove extracted files #
##########################

echo -e "${CYAN}Info:${COLOR_OFF} Removing extracted file"

rm -rf ${ICEORYX_TEMP_DIR}
