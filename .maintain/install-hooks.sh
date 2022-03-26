touch .git/hooks/pre-commit
echo "#!/bin/sh

make clean
" > .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

touch .git/hooks/pre-push
echo "#!/bin/sh

make prepush
" > .git/hooks/pre-push
chmod +x .git/hooks/pre-push