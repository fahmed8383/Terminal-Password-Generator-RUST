# Terminal Password Generator

## About

This is a terminal based password generator written in Rust. It supports commands such as generating a new random password, getting a password,
deleting a password, and returning a list of all saved password names.

The passwords are saved in a json file and are encrypted using a shift cipher which takes in a user specified key to encrypt and decrypt the password.

**Not having a modern AES standard encryption takes away some of the security of this application but since the password is randomly generated gibberesh,
decrypting with a brute force attack would still be extremely difficult.**

**Still, this application should not be used to store important passwords and other sources like bitwarden or lastpass offer better protection.**

## Usage

Once the command has been installed it can be used with the following flags:

### Generate password with name test

If passgeni is not set, this commands asks you for the key, you need to use the same key to decrypt the password later.

`passgen -g test`

### Delete password with name test

`passgen -d test`

### Get password with name test

If passgeni is not set, this commands asks you for the key, this is the same key you used to generate the password.

`passgen test`

### List names of all saved passwords

`passgen --list`

### List all possible flags that this command offers

`passgen --flags`

**Optionally an additional passgeni command can be added to set a terminal session wide key so you do not need to keep re-entering it. This command is explained later**

## Installation

To be able to install this command you will need to have Rust (Programming language) installed, or specifically the Rust compiler and Cargo package manager.

Once Rust has been installed, follow these steps on Linux:

1. `cd installation/`
2. `chmod u+x ./install.sh`
3. run `./install.sh`
4. Enter the directory that you want to install the command in
5. Add the directory that you installed the command in to your $PATH

The directory can be added to your PATH in the .bashrc file (or other alternative shell config) if you want to permenantly keep it in PATH even after terminal session is closed.

**For windows, similar commands can be used to install but ./install.sh script would need to be ported over to a batch script.**

## Adding Optional Command (passgeni)

Optionally a `passgeni` command can also be added to your .bashrc (or other alternative shell config). This command will ask you for your key and save it as an environment
variable that lasts until the terminal is closed. As a result the passgen generate and get password file will use this environment variable to get the key instead of asking
you for a key.

To add passgeni, add the following to your shell config file:

```
passgeni() {
        echo "Enter key to encrypt password with: "
        read -s key

        echo "Re-enter key: "
        read -s key_confirm

        if [ $key != $key_confirm ]
        then
        echo "Keys do not match. Please try again"
        exit
        fi

        export PASSGEN_KEY=$key
}
```
