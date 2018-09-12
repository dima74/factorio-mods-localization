# Service for easy configuration factorio mods translation on Crowdin

# Short description
TODO

# Motivation
TODO

# Usage
TODO

# How it works
0. Mod author has mod repository on GitHub
1. Mod author installs GitHub app (for his mod repository)
2. Our service creates subdirectory in our Crowdin project and upload original strings and existing translations into it
3. Every week our service take strings from Crowdin and make commit to GitHub repository (if there are any changes)
4. Every time original (locale/en) strings are changed, our service changes appropriate strings on Crowdin 

# Things to consider
* в идеале сделать для каждого мода отдельный проект на Crowdin
    - надо уточнить, доступно ли для open source license неограниченное число проектов
    - подумать над тем, чтобы для каждого мода отправлять заявку на open source license
* папка с модом может быть подпапкой репозитория
* репозиторий может содержать несколько модов (каждый в своей подпапке)
    - как обрабатывать переименование подпапок?
* могут ли меняться идентификаторы строк (при сохранении исходной строки и переводов)?
* как обрабатывать изменение исходной строки?
* стоит ли разрешать редактирование переводов на GitHub (с последующим переносом на Crowdin (и как этот перенос реализовать в случае конфликтов))?
* что будет если автор мода удалит приложение и заново установит?
