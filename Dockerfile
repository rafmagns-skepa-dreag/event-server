FROM python:3.8 as python-base

    # python
ENV PYTHONUNBUFFERED=1 \
    # prevents python creating .pyc files
    PYTHONDONTWRITEBYTECODE=1 \
    \
    # pip
    PIP_NO_CACHE_DIR=off \
    PIP_DISABLE_PIP_VERSION_CHECK=on \
    PIP_DEFAULT_TIMEOUT=100 \
    \
    # poetry
    # https://python-poetry.org/docs/configuration/#using-environment-variables
    POETRY_VERSION=1.0.3 \
    # make poetry install to this location
    POETRY_HOME="/opt/poetry" \
    # make poetry create the virtual environment in the project's root
    # it gets named `.venv`
    POETRY_VIRTUALENVS_IN_PROJECT=false \
    # do not ask any interactive question
    POETRY_NO_INTERACTION=1 \
    POETRY_VIRTUALENVS_CREATE=false

# prepend poetry and venv to path
ENV PATH="$POETRY_HOME/bin:$PATH"

#RUN apk add --no-cache curl g++ make build-utils
RUN apt-get update \
    && apt-get install --no-install-recommends -y \
        # deps for installing poetry
        curl \
        # deps for building python deps
        build-essential net-tools

# install poetry - respects $POETRY_VERSION & $POETRY_HOME
RUN curl -sSL https://raw.githubusercontent.com/sdispater/poetry/master/get-poetry.py | python

# copy project requirement files here to ensure they will be cached.
WORKDIR /app
COPY poetry.lock pyproject.toml ./

RUN poetry install --no-dev
RUN rm -rf ~/.cache


# `development` image is used during development / testing
FROM python-base as development
ENV FASTAPI_ENV=development
# quicker install as runtime deps are already installed
RUN poetry install
COPY . .
EXPOSE 8000
CMD ["uvicorn", "--host", "0.0.0.0", "main:app"]


# `production` image used for runtime
FROM python-base as production
ENV FASTAPI_ENV=production
COPY . .
WORKDIR /app
CMD ["gunicorn", "-k", "uvicorn.workers.UvicornWorker", "main:app"]
