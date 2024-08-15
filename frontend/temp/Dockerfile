# syntax=docker/dockerfile:1

ARG NODE_VERSION=22.6.0

FROM node:${NODE_VERSION}-alpine AS base

WORKDIR usr/src/app

FROM base AS deps

FROM deps AS build


RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=package-lock.json,target=package.json \
    --mount=type=cache,target=/root/.npm \
    npm ci

RUN npm install

RUN npm i -g serve

COPY . .

RUN npm run build

FROM base AS final

ENV NODE_ENV=production

USER node

COPY --from=deps /usr/src/app/node_modules ./node_modules
COPY --from=build /usr/src/app/build ./build

EXPOSE 3000

CMD [ "serve", "-s", "dist" ]
