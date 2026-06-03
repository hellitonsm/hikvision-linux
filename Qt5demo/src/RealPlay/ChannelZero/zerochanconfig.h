#ifndef ZEROCHANCONFIG_H
#define ZEROCHANCONFIG_H

#include <QDialog>
#include "ui_zerochanconfig.h"
#include "HCNetSDK.h"

class ZeroChanConfig : public QDialog
{
    Q_OBJECT

public:
    ZeroChanConfig(int userID, QWidget *parent = 0);
    ~ZeroChanConfig();

    bool loadZeroChanConfig();
    bool saveZeroChanConfig();

private slots:
    void on_pushButton_get_clicked();
    void on_pushButton_set_clicked();
    void on_pushButton_cancel_clicked();

private:
    Ui::ZeroChanConfigClass ui;
    int m_userID;
};

#endif // ZEROCHANCONFIG_H
