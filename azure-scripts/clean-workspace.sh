file="functions/functions.txt"
functions=`cat $file`

for f in $functions; do
    mv "$f" "functions" 
done

rm handler