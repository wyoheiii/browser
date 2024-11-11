#! /bin/bash -xe
HOME_PATH=$PWD
TARGET_PATH=$HOME_PATH"/build"
OS_PATH=$TARGET_PATH"/wasabi"
APP_NAME="browser"
MAKEFILE_PATH=$HOME_PATH"/Makefile"

if [ -d $TARGET_PATH ]
then
  echo $TARGET_PATH" exists"
else
  echo $TARGET_PATH" does not exist"
  mkdir $TARGET_PATH
fi

if [ -d $OS_PATH ]
then
  echo $OS_PATH" exists"
  echo "pulling new changes...."
  cd $OS_PATH
  git pull origin for_saba
else
  echo $OS_PATH" does not exist"
  echo "cloning wasabi project...."
  cd $TARGET_PATH
  git clone --branch for_saba git@github.com:hikalium/wasabi.git
fi

cd $HOME_PATH

if [ ! -f $MAKEFILE_PATH ]
then
  echo "downloading Makefile...."
  wget https://raw.githubusercontent.com/hikalium/wasabi/main/external_app_template/Makefile
fi

make build
$OS_PATH/scripts/run_with_app.sh ./target/x86_64-unknown-none/release/$APP_NAME