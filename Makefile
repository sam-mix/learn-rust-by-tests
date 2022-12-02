# 运行socket demo
.PHONY: socket-demo
socket-demo:
	@cd ./socket-demo && go run main.go && cd ..

# go workspace sync
# go 工作目录同步 创建go mod后使用
# 或者在不存在go.work.sum 时使用
.PHONEY: sync
sync:
	@go work sync
