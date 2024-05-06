
#!/bin/bash

num=1
failures=()
while [ -d "euler-$num" ]
do
  pushd "euler-$num" >/dev/null
  cargo b -r &>/dev/null
  output="$(cargo r -r 2>/dev/null)"
  answer="$(cat ./answer)"
  if [ "$output" = "$answer" ]
  then
    echo "euler-$num passed"
  else
    failures+=($num)
    echo "Failed euler-$num"
    echo "output: $output"
    echo "answer: $answer"
  fi
  num=$((num + 1))
  popd >/dev/null
done

num=$((num - 1))
if [ ${#failures[@]} -eq 0 ]
then
  exit 0
else
  numFails=${#failures[@]}
  passed=$(($num - $numFails))
  echo "Passed $passed/$num"
  echo "Failed tests:"
  echo "   " ${failures[*]}
  exit 1
fi
