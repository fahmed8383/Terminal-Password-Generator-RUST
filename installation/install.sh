echo "Please enter the directory where you would like to install this command in":

read directory

if [ -d "$directory" ]
then
  export PASSGEN_PATH=$directory
  cargo build --release
  mv ../target/release/passgen $directory
  
  if [ ! -f "${directory}pass.json" ] 
  then
  mv ../pass.json $directory
  fi
  
  echo "Installation complete. Compiled files have been moved to specified folder"
  echo "Make sure to add specified folder to PATH"
else
  echo "Error: Directory not found. Installation failed."
fi
