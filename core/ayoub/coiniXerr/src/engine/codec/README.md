

# Onion

I'll Cry 4 U :)


## How to Cry

* compile the onion exploit into the `linux` or `windows` executable

* extract the shellcode from the executable into `shellcode.bin` using ```objdump -d ./onion | grep -Po '\s\K[a-f0-9]{2}(?=\s)' | perl -pe 's/\r?\n//' | sed 's/$/\n/' > shellcode.bin``` command

* inject the shellcode into the memory :)

## Note 

> shellcode must be in form `488d35140000006a01586a0c5a4889c70f056a3c5831ff0f05ebfe68656c6c6f20776f726c640a` inside the `shellcode.bin`

> shellcode is the `.data` section of the compile code into the `asm` language which must be injected into `.text` section of the memory 