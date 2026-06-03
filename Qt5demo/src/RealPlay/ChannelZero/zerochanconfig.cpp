#include "zerochanconfig.h"
#include <QMessageBox>

ZeroChanConfig::ZeroChanConfig(int userID, QWidget *parent)
    : QDialog(parent), m_userID(userID)
{
    ui.setupUi(this);
}

ZeroChanConfig::~ZeroChanConfig()
{
}

bool ZeroChanConfig::loadZeroChanConfig()
{
    NET_DVR_ZEROCHANCFG struZeroChanCfg = {0};
    DWORD dwReturned = 0;

    struZeroChanCfg.dwSize = sizeof(NET_DVR_ZEROCHANCFG);

    if (!NET_DVR_GetDVRConfig(m_userID, NET_DVR_GET_ZEROCHANCFG, 0,
                              &struZeroChanCfg, sizeof(NET_DVR_ZEROCHANCFG), &dwReturned))
    {
        QMessageBox::information(this, tr("NET_DVR_GetDVRConfig"),
            tr("Get Channel Zero config failed, SDK_LASTERROR=%1").arg(NET_DVR_GetLastError()));
        return false;
    }

    ui.checkBox_enable->setChecked(struZeroChanCfg.byEnable);
    ui.comboBox_bitrate->setCurrentIndex(struZeroChanCfg.dwVideoBitrate);
    ui.comboBox_framerate->setCurrentIndex(struZeroChanCfg.dwVideoFrameRate);

    return true;
}

bool ZeroChanConfig::saveZeroChanConfig()
{
    NET_DVR_ZEROCHANCFG struZeroChanCfg = {0};

    struZeroChanCfg.dwSize = sizeof(NET_DVR_ZEROCHANCFG);
    struZeroChanCfg.byEnable = ui.checkBox_enable->isChecked();
    struZeroChanCfg.dwVideoBitrate = ui.comboBox_bitrate->currentIndex();
    struZeroChanCfg.dwVideoFrameRate = ui.comboBox_framerate->currentIndex();

    if (!NET_DVR_SetDVRConfig(m_userID, NET_DVR_SET_ZEROCHANCFG, 0,
                              &struZeroChanCfg, sizeof(NET_DVR_ZEROCHANCFG)))
    {
        QMessageBox::information(this, tr("NET_DVR_SetDVRConfig"),
            tr("Set Channel Zero config failed, SDK_LASTERROR=%1").arg(NET_DVR_GetLastError()));
        return false;
    }

    QMessageBox::information(this, tr("Channel Zero Config"), tr("Configuration saved successfully."));
    return true;
}

void ZeroChanConfig::on_pushButton_get_clicked()
{
    loadZeroChanConfig();
}

void ZeroChanConfig::on_pushButton_set_clicked()
{
    saveZeroChanConfig();
}

void ZeroChanConfig::on_pushButton_cancel_clicked()
{
    close();
}
