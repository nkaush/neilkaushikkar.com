file="functions/functions.txt"
functions=`cat $file`

for f in $functions; do
    mv "functions/$f" "." 
done