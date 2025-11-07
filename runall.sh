set -euo pipefail
# pyenv activate svk_env

for dir in */
do
  echo "${dir}"
  cd "${dir}"
  echo "executing run_benches in ${dir}"
  ../run_benches.sh
  echo "${dir} benched, continuing"
  cd ..
#   pwd
done

