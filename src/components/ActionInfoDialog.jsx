import { DatePicker } from "antd";
import dayjs from "dayjs";

function ActionInfoDialog({
  showActionInfoDialog,
  actionForm,
  setActionForm,
  handleActionInfoInputChange,
  saveActionInfo,
  setShowActionInfoDialog,
}) {
  if (!showActionInfoDialog) return null;

  return (
    <div
      className="dialog-overlay"
      onClick={() => setShowActionInfoDialog(false)}
    >
      <div className="dialog" onClick={(e) => e.stopPropagation()}>
        <div className="dialog-header">
          <h3>操作笔记</h3>
        </div>
        <div className="dialog-content">
          <div className="time-input-container">
            <DatePicker
              showTime
              format="YYYY年MM月DD日 HH:mm"
              placeholder="请选择日期和时间"
              value={
                actionForm.actionTime ? dayjs(actionForm.actionTime) : null
              }
              onChange={(date, dateString) => {
                setActionForm((prev) => ({
                  ...prev,
                  actionTime: date ? date.format("YYYY-MM-DDTHH:mm") : "",
                }));
              }}
              className="antd-datetime-picker"
            />
          </div>
          <textarea
            name="actionInfo"
            value={actionForm.actionInfo}
            onChange={handleActionInfoInputChange}
            className="action-info-textarea"
            rows="6"
            cols="50"
            placeholder="请输入操作信息..."
          ></textarea>
        </div>
        <div className="dialog-actions">
          <button className="btn-confirm" onClick={saveActionInfo}>
            保存
          </button>
          <button
            className="btn-cancel"
            onClick={() => {
              setActionForm({
                stockActionId: actionForm.stockActionId,
                actionTime: "",
                actionInfo: "",
              });
            }}
          >
            清空
          </button>
          <button
            className="btn-cancel"
            onClick={() => setShowActionInfoDialog(false)}
          >
            关闭
          </button>
        </div>
      </div>
    </div>
  );
}

export default ActionInfoDialog;
