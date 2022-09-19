# Notifyer

Rustで作ったRaspberry Piでも動くGitHubの更新状況をCronで指定した時間にSlackで通知するMiniアプリ


## 機能

### GitHubを更新してないときに通知を送る

```shell
./notifyer notify
```

![Notifyer](readme_images/notifyer.png)

### 1日のコミットのサマリー

```shell
# 同日のコミット状況を通知する
./notifyer summary

# 前日の場合 (12時を回ったあとに通知するときに便利)
./notifyer summary_yesterday
```

![Summary](readme_images/summary.png)

## 設定

### I. Cross compileする

Raspberry Pi用にCross compileする

### II. Slackの準備

### III. Cron/deployの準備
