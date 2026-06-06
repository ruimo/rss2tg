pipeline {
    agent any
    
    triggers {
        // 毎時0分に実行
        cron('0 * * * *')
    }
    
    environment {
        // Jenkinsの認証情報から環境変数を設定
        TELEGRAM_TOKEN = credentials('telegram-token')
        CHAT_ID = credentials('telegram-chat-id')
        RSS_URL = credentials('rss-url')
        DB_PATH = "${WORKSPACE}/.rss-cache/rss_data.db"
    }
    
    stages {
        stage('Setup') {
            steps {
                script {
                    echo "Setting up RSS monitor..."
                    sh 'mkdir -p .rss-cache'
                }
            }
        }
        
        stage('Download Binary') {
            steps {
                script {
                    echo "Downloading latest release..."
                    sh '''
                        # 最新のリリースからバイナリをダウンロード
                        LATEST_RELEASE=$(curl -s https://api.github.com/repos/ruimo/rss2tg/releases/latest | grep "browser_download_url.*tar.gz" | grep -v ".sha256" | cut -d '"' -f 4)
                        echo "Downloading from: $LATEST_RELEASE"
                        curl -L -o rss2tg.tar.gz "$LATEST_RELEASE"
                        tar -xzf rss2tg.tar.gz
                        chmod +x rss2tg
                        ls -lh rss2tg
                    '''
                }
            }
        }
        
        stage('Run RSS Monitor') {
            steps {
                script {
                    echo "Running RSS monitor..."
                    dir('.rss-cache') {
                        sh '../rss2tg'
                    }
                }
            }
        }
        
        stage('Show Database Info') {
            steps {
                script {
                    sh '''
                        if [ -f .rss-cache/rss_data.db ]; then
                            echo "Database size: $(du -h .rss-cache/rss_data.db)"
                        fi
                    '''
                }
            }
        }
    }
    
    post {
        always {
            // クリーンアップ（バイナリは削除するが、DBは保持）
            sh 'rm -f rss2tg rss2tg.tar.gz'
        }
        success {
            echo 'RSS monitor completed successfully'
        }
        failure {
            echo 'RSS monitor failed'
        }
    }
}

// Made with Bob
