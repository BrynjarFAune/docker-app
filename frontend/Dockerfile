FROM node:22.6.0-alpine AS base

WORKDIR /usr/src/app

FROM base AS deps

RUN --mount=type=bind,source=package.json,target=package.json \
    --mount=type=bind,source=package-lock.json,target=package-lock.json \
    --mount=type=cache,target=/root/.npm \
    npm ci

FROM deps AS build

COPY . .

RUN npm run build

FROM base AS final

ENV NODE_ENV production

USER node

COPY package.json .

COPY --from=deps usr/src/app/node_modules ./node_modules
COPY --from=build usr/src/app/.svelte-kit ./.svelte-kit

EXPOSE 5173

CMD ["npm", "run", "dev"]
