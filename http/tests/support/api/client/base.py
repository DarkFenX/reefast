from __future__ import annotations

from typing import TYPE_CHECKING

import requests

from tests.support.response import Response

if TYPE_CHECKING:
    from tests.support.log import LogReader
    from tests.support.request import Request


class ApiClientBase:

    def __init__(self, *, port: int, log_reader: LogReader, **kwargs):
        super().__init__(**kwargs)
        self.__session: requests.Session = requests.Session()
        self.__base_url: str = f'http://localhost:{port}'
        self.__log_reader: LogReader = log_reader

    def send_prepared(self, *, req: Request) -> Response:
        response = self.__session.send(req)
        return Response(response=response)

    @property
    def _base_url(self) -> str:
        return self.__base_url

    @property
    def _log_reader(self) -> LogReader:
        return self.__log_reader
