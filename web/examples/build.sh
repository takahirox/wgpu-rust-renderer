for dir in $(ls -d ./*/ | grep -v icons | grep -v utils)
do
  cd $dir
  bash build.sh
  cd -
done
