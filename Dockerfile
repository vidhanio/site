FROM node:17.1.0-alpine

WORKDIR /usr/app

COPY . .

RUN yarn install --frozen-lockfile --production
RUN yarn add --dev typescript @types/react

RUN yarn build

CMD [ "yarn", "start" ]