# NamuPPuli
나무위키의 하이퍼링크 구문([[Rust]])을 분석하여 여러가지 작업을 행하는 프로그램 입니다.

# 기능
- [x] 빈도
- [x] 관계망 추출
- [ ] 이웃 그래프 생성
- [ ] ...

# 빌드
간단하게 기본 명령어로 빌드가 가능합니다.
``` bash
cargo build --release
```

# 사용법
먼저 [나무위키:데이터베이스 덤프](https://namu.wiki/w/%EB%82%98%EB%AC%B4%EC%9C%84%ED%82%A4:%EB%8D%B0%EC%9D%B4%ED%84%B0%EB%B2%A0%EC%9D%B4%EC%8A%A4%20%EB%8D%A4%ED%94%84)를 받습니다.

아래 명령어를 이용해 관계망 추출합니다.[^1][^2]
``` bash
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
![image](https://user-images.githubusercontent.com/8307128/193088031-5071d7fd-8f9f-4cd5-9193-aa2597fd5cfe.png)

[^1]:i7-4790k기준으로 약 6~7분이 소모됩니다
[^2]:램 최소 8GB이상을 권고합니다
[^3]:어차피 이웃검색중에 자동적으로 정렬이 이뤄집니다.
