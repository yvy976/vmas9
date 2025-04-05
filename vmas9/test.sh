for files in tests/*.v; do
    base=$(basename "$files" .v)
    asm="tests/$base.asm"
    dotnet run "$asm" h.v

    hexdump -C h.v > trash.txt
    hexdump -C "$files" > trash2.txt

    c=$(diff trash.txt trash2.txt | wc -l)
    if [ "$c" -ne 0 ]; then
        echo "mismatch in $files"
    fi
done


