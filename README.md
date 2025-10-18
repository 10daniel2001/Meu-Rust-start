echo "# Meu-Rust-start" >> README.md
# EN: Creates a README.md file and writes the title “Meu-Rust-start” inside it.
# PT: Cria um arquivo README.md e escreve o título “Meu-Rust-start” dentro dele.

git init
# EN: Initializes a new Git repository in the current folder.
# PT: Inicializa um novo repositório Git na pasta atual.

git add README.md
# EN: Adds the README.md file to the staging area (preparing it to be committed).
# PT: Adiciona o arquivo README.md à área de stage (preparando para o commit).

git commit -m "first commit"
# EN: Creates the first commit with the message “first commit”.
# PT: Cria o primeiro commit com a mensagem “first commit”.

git branch -M main
# EN: Renames the default branch to “main”.
# PT: Renomeia a branch padrão para “main”.

git remote add origin https://github.com/10daniel2001/Meu-Rust-start.git
# EN: Links the local repository to the remote GitHub repository.
# PT: Vincula o repositório local ao repositório remoto no GitHub.

git push -u origin main
# EN: Sends (pushes) the local code to the remote repository, setting “main” as the default upstream branch.
# PT: Envia o código local para o repositório remoto, definindo “main” como branch principal padrão.
