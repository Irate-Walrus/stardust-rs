import sys

def djb2_hash(buf: bytes) -> int:
    hash_value = 5381  # Start with the u32 hash
    for i, byte in enumerate(buf):
        prev = hash_value
        if byte == 0x0:
            continue
        hash_value = (hash_value << 5) + hash_value + byte  # hash * 33 + byte
        hash_value = hash_value & 0xffffffff # Ensure the result fits in a 32-bit unsigned integer
        print(f'{chr(byte)}:\t {hex(prev)}   \t+\t{hex(byte)} =\t{hex(hash_value)}')
    return hash_value

if len(sys.argv) < 2:
    print("[!] please provide input string.")
    exit()

input_str = sys.argv[1]
hash_value = djb2_hash(input_str.encode('utf-8'))
print(f"DJB2 hash for '{input_str}' (hex): {hex(hash_value)}")
