# CS-128-honors-final-project - Poke Cipherers

Group member names and NetIDs:

Ian Qiu (ianqiu2)

Jaydeen Jaramillo (jjara23)

Andrew Liang (alian24)

# Project Introduction

The user will input a string that they would want to encrypt and another string with their favorite Pokémon. Then we will perform a Vigenère cipher on the string and return it to the user. We chose to work on this because it would be fun to learn about a Vigenère cipher, while limiting the cipher to only work with Pokémon names as the keys to trick cryptography lovers.
Technical Overview

# Main Technical Components

Create an encryption function of the Vigenere Cipher.
Create a method to encrypt using the Vigenere Cipher.
Find a way to store and limit only to the first 151 Pokémon names.
There is a Rust crate called pokemon-rs that has all the Pokémon names.
Decryption function of the Vigenere Cipher.
Create a method to decrypt the given word using a keyword(Pokemon Name).
User Interface for inputting the word and key for encryption and decryption.

The Vigenere cipher is just repeated Caesar ciphers, so we will make a Caesar cipher helper function.
You can learn more about the Vigenere cipher here: https://en.wikipedia.org/wiki/Vigen%C3%A8re_cipher

# Checkpoint 1

Finish the encryption and decryption functions by checkpoint 1 without considering user input and UI.

# Checkpoint 2 

Design the user interface, allowing for user input of their word and Pokémon.

# Final Submission

Consider error handling as well as any additional features that we think of at this time.

# Possible Challenges

Out of the MPs and HWs so far in this course, we haven’t had any practice with user input and UI design, so that could potentially be a challenge.


# References

Inspired by the Vigenère cipher by Giovan Battista Bellaso
