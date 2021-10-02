for dir in $(ls -d ./*/ | grep -v icons)
do
  cd $dir
  bash build.sh
  cd -
done
