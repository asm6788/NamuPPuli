# NamuPPuli
나무위키의 하이퍼링크 구문([[Rust]])을 분석하여 여러가지 작업을 행하는 프로그램 입니다.

![image](https://user-images.githubusercontent.com/8307128/193088031-5071d7fd-8f9f-4cd5-9193-aa2597fd5cfe.png)

# 기능
- [x] 빈도
- [x] 관계망 추출
- [x] 이웃 그래프 생성
- [ ] ...

# 빌드
간단하게 기본 명령어로 빌드가 가능합니다.
``` bash
cargo build --release
```
# 주의점
키워드망 분석중 노이즈가 발생하기 쉬운 날짜 문서(~세기,~년,~월 ~일) 그리고 기본적으로 가중치가 3미만인 연결은 무시됩니다.(-t로 변경 가능)

# 사용법
먼저 [나무위키:데이터베이스 덤프](https://namu.wiki/w/%EB%82%98%EB%AC%B4%EC%9C%84%ED%82%A4:%EB%8D%B0%EC%9D%B4%ED%84%B0%EB%B2%A0%EC%9D%B4%EC%8A%A4%20%EB%8D%A4%ED%94%84)를 받습니다.

아래 명령어를 이용해 관계망 추출합니다.[^1][^2]
``` bash
namuPPuli -n "나무위키덤프경로" -c -s > network.csv
```
굳이 관계망 파일의 가중치 정렬이 필요하지 않으시다면 ``-s``를 떼셔도 무방합니다.[^3]

아래 명령어로 관계망에서 이웃들을 검색하실 수 있습니다.
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
| `-t / --threshold` | 키워드망 최소 가중치 |
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
현재 지원하는 프리셋은 아래와 같습니다. 동시에 여러개를 사용하실 수 있습니다.
| 프리셋 | 내용 |
| --- | --- |
| [나라] | "대한민국", "미국", "영국", "프랑스", "독일", "이탈리아", "중국", "러시아", "일본", "북한", "소련" |
| [대한민국 대통령] | "이승만", "윤보선", "박정희", "최규하", "전두환", "노태우", "김영삼", "김대중", "노무현", "이명박", "박근혜", "문재인", "윤석열" |
| [해외 정치인] | "조 바이든", "도널드 트럼프", "버락 오바마", "기시다 후미오", "스가 요시히데", "아베 신조", "올라프 숄츠", "앙겔라 메르켈", "리즈 트러스", "보리스 존슨", "테레사 메이", "에마뉘엘 마크롱", "프랑수아 올랑드", "볼로디미르 젤렌스키", "블라디미르 푸틴", "시진핑", "후진타오", "김일성", "김정일", "김정은" |

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
