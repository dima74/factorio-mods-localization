* Add card to heroku to increase free hours to 1000
* Create another heroku account and configure another dyno, so it will ping main dyno every minute, and vice versa
* дописать README
* подумать нужна ли нам отдельная страница для перенаправления после установки приложения
* Webstorm pullrequest autocomplete for values.map(value => )
* Custom localized badge (via crowdin api)

## Things to consider
* в идеале сделать для каждого мода отдельный проект на Crowdin
    - надо уточнить, доступно ли для open source license неограниченное число проектов
    - подумать над тем, чтобы для каждого мода отправлять заявку на open source license
* папка с модом может быть подпапкой репозитория
* репозиторий может содержать несколько модов (каждый в своей подпапке)
    - как обрабатывать переименование подпапок?
* могут ли меняться идентификаторы строк (при сохранении исходной строки и переводов)?
* стоит ли разрешать редактирование переводов на GitHub (с последующим переносом на Crowdin (и как этот перенос реализовать в случае конфликтов))?
* что будет если автор мода удалит приложение и заново установит?
