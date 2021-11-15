FROM node:17.1.0-alpine

WORKDIR /app
COPY . .

RUN yarn install
RUN yarn build
CMD [ "yarn", "start" ]