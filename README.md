namuPPuli -n "나무위키덤프경로" -c -s > network.csv
```
굳이 관계망 파일의 가중치 정렬이 필요하지 않으시다면 ``-s``를 떼셔도 무방합니다.[^3]

아래 명령어로 관계망에서 이웃들을 검색하실수있습니다.
``` bash
namuPPuli -p network.csv
```
이것도 가능합니다.
``` bash
namuPPuli -n "나무위키덤프경로"
```
빈도분석은 -f 옵션으로 수행할수있습니다.

## 옵션
| 옵션 | 설명 | 예시 |
| --- | --- | --- |
| `-n / --nanum-db` | 나무위키 데이터베이스 경로 |
| `-p / --parsed-db` | 나무위키 키워드망 덤프 경로 |
| `-c / --csv-export` | 키워드망/빈도분석을 csv 형식으로 출력 |
| `-d / --dot-export` | 키워드망을 dot 형식으로 출력 |
| `-D / --neighbor-dot-export` | 검색된 이웃들을 dot 형식으로 출력 |
| `--stopword` | 이 단어들의 이웃을 검색하지 않습니다 | --stopword "[나라],치킨,피자"
| `-f / --frequency` | 키워드 빈도 분석을 수행합니다 |
| `-s / --sort` | 키워드망/빈도분석을 가중치순으로 정렬합니다 |
| `--depth` | 얼마나 재귀적으로 이웃을 탐색할지 정합니다(기본값 1) | --depth 2 |
| `-h / --help` | 도움말 |
| `--hostname` | gephi 서버 주소(기본값 localhost:8080) | --hostname localhost:8080 |
| `--workspace` | gephi의 Workspace 번호를 지정합니다 | --workspace 1 |

### stopword 프리셋
현재 지원하는 프리셋은 아래와 같습니다. 동시에 여러개를 사용하실수 있습니다
| 프리셋 | 내용 |
| --- | --- |
| [나라] | "대한민국", "영국", "프랑스", "중국", "일본", "북한", "소련" |

### Gephi GraphStreaming 사용
[GraphStreaming API](https://github.com/gephi/gephi/wiki/GraphStreaming)를 이용하여 Gephi로 실시간으로 그래프를 생성합니다.
-D 옵션과 양립할수없습니다.

1. 도구-플러그인-사용 가능한 플러그인 - 검색에 Graph Streaming입력 - 설치
2. Workspace를 생성합니다
3. 왼족하단에 Streaming 탭 생성확인
4. Master-Master Server 우클릭-Start
5. --workspace 옵션에 Workspace 옆에 적힌숫자를 입력합니다. 만약, 다른컴퓨터로 송신한다면 --hostname을 변경하셔야합니다.
6. Layout의 Scaling등 크기에 관련된 변수를 수정하시고(100~1000정도), 평소처럼 그래프 생성하듯이 하시면 됩니다.

[^1]:i7-4790K기준으로 약 17초/M1기준으로 10초 소모됩니다.
[^2]:램 최소 8GB이상을 권고합니다
[^3]:어차피 이웃검색중에 자동적으로 정렬이 이뤄집니다.
