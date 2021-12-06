FROM node:17.2.0-alpine

WORKDIR /app
COPY . .

RUN yarn install --frozen-lockfile
RUN yarn build
CMD [ "yarn", "start" ]