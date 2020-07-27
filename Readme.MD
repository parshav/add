## Add
Will add numbers

```shell script
./add [ARGS]
```

eg.

```shell script
./add 10 12.2
Addition : 22.2
```

With xargs from file output :

```shell script
cat file.txt | xargs ./add
Addition : 22.2
```