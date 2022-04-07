FROM node:16.14.2-alpine3.15

ENV DIR /usr/src/app
RUN mkdir -p ${DIR}
WORKDIR ${DIR}

RUN apk add dumb-init make

ENTRYPOINT ["/usr/bin/dumb-init", "--"]

RUN npm install -g node-watch typescript sass esbuild

COPY package.json package.json
COPY package-lock.json package-lock.json
COPY tsconfig.json tsconfig.json

RUN npm install
RUN npm link node-watch

COPY Makefile Makefile
COPY server.js server.js
COPY watcher.js watcher.js

## Any change in the src folder invalidates the cache of this step
##  but the dependencies are still cached
COPY src src
