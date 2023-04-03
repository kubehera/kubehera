package user

import (
	"time"

	"gorm.io/gorm"
)

type User struct {
	gorm.Model
	ID        int64 `gorm:"primaryKey" json:"id"`
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt gorm.DeletedAt `gorm:"index"`
	OauthType string         `gorm:"oauth_type" json:"oauth_type"`
	Login     string         `gorm:"login" json:"login"`
	Name      string         `gorm:"name" json:"name"`
	Email     string         `gorm:"email" json:"email"`
	AvatarUrl string         `gorm:"avatar_url" json:"avatar_url"`
	IsAdmin   bool           `gorm:"is_admin" json:"is_admin"`
}
